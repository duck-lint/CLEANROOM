use semantic_traversal_core::{
    ProjectionActivationAccess, ProjectionActivationAccessFailure, ProjectionActivationProbe,
    ProjectionActivationProbeResult, ProjectionActivationProbeSourceKind, SemanticSpaceProjection,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ScriptedProjectionActivationAccess {
    pub results: Vec<(ProjectionActivationProbe, ProjectionActivationProbeResult)>,
    pub failures: Vec<(ProjectionActivationProbe, ProjectionActivationAccessFailure)>,
    pub declared_modes: Vec<(String, String, ProjectionActivationProbeSourceKind)>,
}

impl ProjectionActivationAccess for ScriptedProjectionActivationAccess {
    fn execute_probe(
        &self,
        _projection: &SemanticSpaceProjection,
        probe: &ProjectionActivationProbe,
    ) -> Result<ProjectionActivationProbeResult, ProjectionActivationAccessFailure> {
        let mut matched_results = self
            .results
            .iter()
            .filter(|(expected, _)| expected == probe);
        let first_result = matched_results.next();
        let duplicate_result = matched_results.next().is_some();

        let mut matched_failures = self
            .failures
            .iter()
            .filter(|(expected, _)| expected == probe);
        let first_failure = matched_failures.next();
        let duplicate_failure = matched_failures.next().is_some();

        let matches = usize::from(first_result.is_some()) + usize::from(first_failure.is_some());
        if duplicate_result || duplicate_failure || matches > 1 {
            return Err(ProjectionActivationAccessFailure {
                context: "duplicate scripted activation probe definition".into(),
            });
        }
        if let Some((_, result)) = first_result {
            return Ok(result.clone());
        }
        if let Some((_, failure)) = first_failure {
            return Err(failure.clone());
        }
        Err(ProjectionActivationAccessFailure {
            context: format!("unexpected activation probe: {probe:?}"),
        })
    }

    fn declared_mode_source(
        &self,
        surface_id: &str,
        mode_name: &str,
    ) -> Option<ProjectionActivationProbeSourceKind> {
        self.declared_modes
            .iter()
            .find(|(surface, mode, _)| surface == surface_id && mode == mode_name)
            .map(|(_, _, source)| source.clone())
    }
}
