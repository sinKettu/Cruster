use std::collections::HashMap;

use super::{traits::{ActiveRuleExecutionContext, BasicContext, WithChangeAction, WithFindAction, WithGetAction, WithSendAction, WithWatchAction}, ActiveRuleContext};
use crate::{audit::{rule_actions::WatchId, types::{CapturesBorders, SendActionResultsPerPatternEntry, SingleCoordinates, SingleSendActionResult}, AuditError, Rule}, http_storage::RequestResponsePair};

impl<'pair_lt, 'rule_lt> BasicContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt>
{
    fn init(rule: &'rule_lt Rule, pair: &'pair_lt RequestResponsePair) -> Self {
        // set initial pair as send action result with index 0
        let initial_send_results: Vec<SendActionResultsPerPatternEntry<'rule_lt>> = vec![
            vec![
                HashMap::from([
                    (
                        "__VERY_INITIAL_PAIR__",
                        SingleSendActionResult {
                            request_sent: pair.request.as_ref().unwrap().clone(),
                            positions_changed: SingleCoordinates {
                                line: 0,
                                start: 0,
                                end: 0
                            },
                            responses_received: vec![pair.response.as_ref().unwrap().clone()]
                        }
                    )
                ])
            ]
        ];

        ActiveRuleContext {
            rule_id: rule.get_id().to_string(),
            pair,
            watch_results: Vec::with_capacity(10),
            watch_succeeded_for_change: false,
            change_results: Vec::with_capacity(10),
            send_results: initial_send_results,
            find_results: Vec::with_capacity(10),
            get_result: Vec::with_capacity(10),
        }
    }

    fn initial_pair(&self) -> &'pair_lt RequestResponsePair {
        self.pair
    }

    fn initial_request(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperRequestWrapper> {
        self.pair.request.as_ref()
    }

    fn initial_response(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperResponseWrapper> {
        self.pair.response.as_ref()
    }

    fn pair_id(&self) -> usize {
        self.pair.index
    }

    fn rule_id(&self) -> &str {
        &self.rule_id
    }
}

impl<'pair_lt, 'rule_lt> WithWatchAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_watch_result(&mut self, res: CapturesBorders) {
        self.watch_results.push(res);
    }

    fn watch_results(&self) -> &Vec<CapturesBorders> {
        &self.watch_results
    }
}

impl<'pair_lt, 'rule_lt> WithChangeAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_change_result(&mut self, res: crate::audit::types::SingleCaptureGroupCoordinates) {
        self.watch_succeeded_for_change = true;
        self.change_results.push(res);
    }

    fn change_results(&self) -> &Vec<crate::audit::types::SingleCaptureGroupCoordinates> {
        &self.change_results
    }

    fn found_anything_to_change(&self) -> bool {
        self.watch_succeeded_for_change
    }
}

impl<'pair_lt, 'rule_lt> WithSendAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_send_result(&mut self, res: crate::audit::types::SendActionResultsPerPatternEntry<'rule_lt>) {
        self.send_results.push(res);
    }

    fn send_results(&self) -> &Vec<crate::audit::types::SendActionResultsPerPatternEntry> {
        &self.send_results
    }
}

impl<'pair_lt, 'rule_lt> WithFindAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn add_find_result(&mut self, res: bool) {
        self.find_results.push(res);
    }

    fn find_results(&self) -> &Vec<bool> {
        &self.find_results
    }

    fn found_anything(&self) -> bool {
        self.find_results.iter().any(|result| { *result })
    }
}

impl<'pair_lt, 'rule_lt> WithGetAction<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn find_action_secceeded(&self, id: usize) -> bool {
        if id >= self.find_results.len() {
            false
        }
        else {
            self.find_results[id]
        }
    }

    fn get_pair_by_id(&self, id: usize) -> Result<&SendActionResultsPerPatternEntry<'rule_lt>, AuditError> {
        if id >= self.send_results.len() {
            return Err(AuditError("Index of Send results is out of bounds".to_string()));
        }

        Ok(&self.send_results[id])
    }

    fn add_empty_result(&mut self) {
        self.get_result.push(None);
    }

    fn add_get_result(&mut self, res: Vec<u8>) {
        self.get_result.push(Some(res));
    }
}

impl<'pair_lt, 'rule_lt> ActiveRuleExecutionContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
    fn make_result(self) -> crate::audit::RuleResult {
        todo!()
    }
}

// impl<'pair_lt, 'rule_lt,> RuleExecutionContext<'pair_lt, 'rule_lt> for ActiveRuleContext<'pair_lt, 'rule_lt> {
//     fn make_result(self) -> crate::audit::RuleResult {
//         todo!()
//     }

//     fn initial_pair(&self) -> &'pair_lt RequestResponsePair {
//         self.pair
//     }

//     fn initial_request(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperRequestWrapper> {
//         self.pair.request.as_ref()
//     }

//     fn initial_response(&self) -> Option<&'pair_lt crate::cruster_proxy::request_response::HyperResponseWrapper> {
//         self.pair.response.as_ref()
//     }

//     fn pair_id(&self) -> usize {
//         self.pair.index
//     }

//     fn add_watch_result(&mut self, res: CapturesBorders) {
//         self.watch_results.push(res);
//     }

//     fn watch_results(&self) -> &Vec<CapturesBorders> {
//         &self.watch_results
//     }

//     fn add_change_result(&mut self, res: crate::audit::types::SingleCaptureGroupCoordinates) {
//         self.change_results.push(res);
//     }

//     fn change_results(&self) -> &Vec<crate::audit::types::SingleCaptureGroupCoordinates> {
//         &self.change_results
//     }

//     fn found_anything_to_change(&self) -> bool {
//         self.watch_succeeded_for_change
//     }

//     fn add_send_result(&mut self, res: crate::audit::types::SendActionResultsPerPatternEntry<'rule_lt>) {
//         self.send_results.push(res);
//     }

//     fn send_results(&self) -> &Vec<crate::audit::types::SendActionResultsPerPatternEntry> {
//         &self.send_results
//     }

//     fn add_find_result(&mut self, res: bool) {
//         self.find_results.push(res);
//     }

//     fn find_results(&self) -> &Vec<bool> {
//         &self.find_results
//     }
// }