use ode_solvers::dopri5::*;
use ode_solvers::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

type State = Vector4<f64>;
type Time = f64;

#[derive(Serialize, Deserialize)]
struct SeirModel {
    recovery_rate: f64,
    reproduction_number: f64,
    infection_rate: f64,
}

impl ode_solvers::System<State> for SeirModel {
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let susceptible = y[0];
        let exposed = y[1];
        let infectious = y[2];
        let _removed = y[3];

        dy[0] = -self.recovery_rate * self.reproduction_number * susceptible * infectious;
        dy[1] = -1.0 * dy[0] - self.infection_rate * exposed;
        dy[2] = self.infection_rate * exposed - self.recovery_rate * infectious;
        dy[3] = self.recovery_rate * infectious;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SeirStateParams {
    initial_susceptible: f64,
    initial_exposed: f64,
    initial_infectious: f64,
    initial_removed: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SeirSolverParams {
    initial_state: SeirStateParams,
    model_params: SeirModel,
    duration: f64,
}

fn writer(f: &mut String, t: &[Time], s: &[State]) {
    f.write_str("{").unwrap();
    for (i, state) in s.iter().enumerate() {
        f.write_fmt(format_args!("\"{}\": ", t[i])).unwrap();
        f.write_str("[").unwrap();
        for val in state.iter() {
            f.write_fmt(format_args!("{}, ", val)).unwrap();
        }
        f.pop();
        f.pop();
        f.write_str("], ").unwrap();
    }
    f.pop();
    f.pop();
    f.write_str("}").unwrap();
}

pub fn solve(params: SeirSolverParams) -> String {
    let system = params.model_params;
    let initial_state = State::new(
        params.initial_state.initial_susceptible,
        params.initial_state.initial_exposed,
        params.initial_state.initial_infectious,
        params.initial_state.initial_removed,
    );
    let mut stepper = Dopri5::new(
        system,
        0.0,
        params.duration,
        1.0,
        initial_state,
        1.0e-10,
        1.0e-10,
    );
    let _res = stepper.integrate().expect("Integration Error :(");
    let mut resp = String::new();

    writer(&mut resp, stepper.x_out(), stepper.y_out());
    resp
}
