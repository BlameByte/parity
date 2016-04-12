// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Simple executive tracer.

use util::{Bytes, Address, U256};
use action_params::ActionParams;
use trace::trace::{Trace, TraceCall, TraceCreate, TraceAction, TraceResult, TraceCreateResult, TraceCallResult};
use trace::Tracer;

/// Simple executive tracer. Traces all calls and creates. Ignores delegatecalls.
#[derive(Default)]
pub struct ExecutiveTracer {
	traces: Vec<Trace>
}

impl Tracer for ExecutiveTracer {
	fn prepare_trace_call(&self, params: &ActionParams) -> Option<TraceCall> {
		Some(TraceCall::from(params.clone()))
	}

	fn prepare_trace_create(&self, params: &ActionParams) -> Option<TraceCreate> {
		Some(TraceCreate::from(params.clone()))
	}

	fn prepare_trace_output(&self) -> Option<Bytes> {
		Some(vec![])
	}

	fn trace_call(&mut self, call: Option<TraceCall>, gas_used: U256, output: Option<Bytes>, depth: usize, subs:
				  Vec<Trace>, delegate_call: bool) {
		// don't trace if it's DELEGATECALL or CALLCODE.
		if delegate_call {
			return;
		}

		let trace = Trace {
			depth: depth,
			subs: subs,
			action: TraceAction::Call(call.expect("Trace call expected to be Some.")),
			result: TraceResult::Call(TraceCallResult {
				gas_used: gas_used,
				output: output.expect("Trace call output expected to be Some.")
			})
		};
		self.traces.push(trace);
	}

	fn trace_create(&mut self, create: Option<TraceCreate>, gas_used: U256, code: Option<Bytes>, address: Address, depth: usize, subs: Vec<Trace>) {
		let trace = Trace {
			depth: depth,
			subs: subs,
			action: TraceAction::Create(create.expect("Trace create expected to be Some.")),
			result: TraceResult::Create(TraceCreateResult {
				gas_used: gas_used,
				code: code.expect("Trace create code expected to be Some."),
				address: address
			})
		};
		self.traces.push(trace);
	}

	fn trace_failed_call(&mut self, call: Option<TraceCall>, depth: usize, subs: Vec<Trace>, delegate_call: bool) {
		// don't trace if it's DELEGATECALL or CALLCODE.
		if delegate_call {
			return;
		}

		let trace = Trace {
			depth: depth,
			subs: subs,
			action: TraceAction::Call(call.expect("Trace call expected to be Some.")),
			result: TraceResult::FailedCall,
		};
		self.traces.push(trace);
	}

	fn trace_failed_create(&mut self, create: Option<TraceCreate>, depth: usize, subs: Vec<Trace>) {
		let trace = Trace {
			depth: depth,
			subs: subs,
			action: TraceAction::Create(create.expect("Trace create expected to be Some.")),
			result: TraceResult::FailedCreate,
		};
		self.traces.push(trace);
	}

	fn subtracer(&self) -> Self {
		ExecutiveTracer::default()
	}

	fn traces(self) -> Vec<Trace> {
		self.traces
	}
}
