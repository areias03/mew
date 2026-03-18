use crate::model::Model;
use good_lp::{
    ProblemVariables, Solution, Solver, SolverModel, solvers::ObjectiveDirection,
    solvers::StaticSolver,
};

type Solution = Vec<f64>;
type SolverResult = Result<Solution, String>;
type SolverError<S> = <<S as Solver>::Model as good_lp::SolverModel>::Error;

pub struct Simulator {
    id: String,
    name: String,
    model: Model,
}

impl Simulator {
    pub fn from_model<T>(model: [T]) -> Self {
        Simulator {
            id: model.id.clone(),
            name: model.name.clone(),
            model,
        }
    }
}

pub struct Solver {
    model: <Simulator>::Model,
}
