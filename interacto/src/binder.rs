/*
 * This file is part of Interacto.
 * Interacto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * Interacto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with Interacto.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod base_binder_builder;

use std::collections::HashMap;

use crate::binding::Binding;

struct InteractionDataType<I> {
    interaction: I,
}

trait Command {}
trait Interaction<D> {}
trait InteractionData {}

/**
 * This alias refers to either an EvenTarget object or a reference to an EvenTarget object.
 * @category Helper
 */
enum Widget<T> {
    EltRef(T),
    Elt(T)
}
trait UnknownWidget {}
enum LogLevel {}
struct UndoHistoryBase {}
struct Logger {}

struct When<D, A> {
    d: D,
    A: A,
}

struct BindingsObserver {}

struct RuleName {}

struct Severity {}

trait Node {}
struct Partial<B> {
    binder: B
}

enum WhenType {}

#[derive(Clone)]
pub struct Binder<C, I, A, D = InteractionDataType<I>>
where
    C: Command,
    I: Interaction<D>,
    D: InteractionData,
{
    first_fn: Option<Box<dyn Fn(&C, &D, &mut A)>>,
    produce_fn: Option<Box<dyn Fn(Option<&D>) -> C>>,
    widgets: Vec<Box<dyn UnknownWidget>>,
    dynamic_nodes: Vec<Box<dyn Node>>,
    using_fn: Option<Box<dyn Fn() -> I>>,
    had_effects_fn: Option<Box<dyn Fn(&C, &D, &A)>>,
    had_no_effect_fn: Option<Box<dyn Fn(&C, &D, &A)>>,
    cannot_exec_fn: Option<Box<dyn Fn(&C, &D, &A)>>,
    end_fn: Option<Box<dyn Fn(&C, &D, &A)>>,
    on_err_fn: Option<Box<dyn Fn(&dyn std::error::Error)>>,
    log_levels: Vec<LogLevel>,
    stop_propagation: bool,
    prev_default: bool,
    binding_name: Option<String>,
    observer: Option<BindingsObserver>,
    undo_history: UndoHistoryBase,
    logger: Logger,
    when_fn_array: Vec<When<D, A>>,
    first_fn_array: Vec<Box<dyn Fn(&C, &D, &mut A)>>,
    end_fn_array: Vec<Box<dyn Fn(&C, &D, &mut A)>>,
    had_effects_fn_array: Vec<Box<dyn Fn(&C, &D, &mut A)>>,
    had_no_effect_fn_array: Vec<Box<dyn Fn(&C, &D, &mut A)>>,
    cannot_exec_fn_array: Vec<Box<dyn Fn(&C, &D, &mut A)>>,
    on_err_fn_array: Vec<Box<dyn Fn(&dyn std::error::Error)>>,
    acc_init: Option<A>,
    linter_rules: HashMap<RuleName, Severity>,
}

impl<C, I, A, D> Binder<C, I, A, D>
where
    C: Command,
    I: Interaction<D>,
    D: InteractionData,
{
    pub fn new(
        undo_history: UndoHistoryBase,
        logger: Logger,
        observer: Option<BindingsObserver>,
        binder: Option<Partial<Binder<C, I, A, D>>>,
        acc: Option<A>,
    ) -> Self {
        let widgets: Vec<Box<dyn UnknownWidget>> = Vec::new();
        let dynamic_nodes: Vec<Box<dyn Node>> = Vec::new();
        let log_levels: Vec<LogLevel> = Vec::new();
        let linter_rules: HashMap<RuleName, Severity> = HashMap::new();
        let stop_propagation = false;
        let prev_default = false;

        let mut instance = Binder {
            first_fn: None,
            produce_fn: None,
            widgets,
            dynamic_nodes,
            using_fn: None,
            had_effects_fn: None,
            had_no_effect_fn: None,
            cannot_exec_fn: None,
            end_fn: None,
            on_err_fn: None,
            log_levels,
            stop_propagation,
            prev_default,
            binding_name: None,
            observer,
            undo_history,
            logger,
            when_fn_array: Vec::new(),
            first_fn_array: Vec::new(),
            end_fn_array: Vec::new(),
            had_effects_fn_array: Vec::new(),
            had_no_effect_fn_array: Vec::new(),
            cannot_exec_fn_array: Vec::new(),
            on_err_fn_array: Vec::new(),
            acc_init: None,
            linter_rules,
        };

        if let Some(b) = binder {
            instance.widgets = b.widgets;
            instance.dynamic_nodes = b.dynamic_nodes;
            instance.log_levels = b.log_levels;
            instance.acc_init = b.acc_init;
            instance.binding_name = b.binding_name;
            instance.observer = b.observer;
            instance.first_fn_array = b.first_fn_array;
            instance.end_fn_array = b.end_fn_array;
            instance.had_effects_fn_array = b.had_effects_fn_array;
            instance.had_no_effect_fn_array = b.had_no_effect_fn_array;
            instance.cannot_exec_fn_array = b.cannot_exec_fn_array;
            instance.on_err_fn_array = b.on_err_fn_array;
            instance.linter_rules = b.linter_rules;
            instance.when_fn_array = b.when_fn_array;
        }

        instance
    }

    fn on<W>(&self, widgets: &[dyn Into<Box<dyn UnknownWidget>>]) -> Self
    where
        W: 'static,
    {
        todo!()
    }

    fn on_dynamic(&self, node: &Widget<dyn Node>) -> Self {
        todo!()
    }

    fn first(&self, func: fn(c: &C, i: &D, acc: &A)) -> Self {
        todo!()
    }

    fn when(&self, func: fn(i: &D, acc: &A) -> bool, mode: WhenType) -> Self {
        todo!()
    }

    fn if_had_effects(&self, func: fn(c: &C, i: &D, acc: &A)) -> Self {
        todo!()
    }

    fn if_had_no_effect(&self, func: fn(c: &C, i: &D, acc: &A)) -> Self {
        todo!()
    }

    fn if_cannot_execute(&self, func: fn(c: &C, i: &D, acc: &A)) -> Self {
        todo!()
    }

    fn end(&self, func: fn(c: &C, i: &D, acc: &A)) -> Self {
        todo!()
    }

    fn log(&self, levels: &[LogLevel]) -> Self {
        todo!()
    }

    fn stop_immediate_propagation(&self) -> Self {
        todo!()
    }

    fn prevent_default(&self) -> Self {
        todo!()
    }

    fn catch(&self, func: fn(ex: &dyn std::error::Error)) -> Self {
        todo!()
    }

    fn name(&self, name: &str) -> Self {
        todo!()
    }

    fn configure_rules(&self, rule_name: &str, severity: Severity) -> Self {
        todo!()
    }

    fn using_interaction<I2: Interaction<D2>, A2, D2: InteractionData>(
        &self,
        func: fn() -> I2,
    ) -> Binder<C, I2, A2, D2> {
        todo!()
    }

    fn to_produce<C2: Command>(&self, func: fn(i: &D) -> C2) -> Binder<C2, I, A, D> {
        todo!()
    }

    fn to_produce_anon(&self, func: fn()) -> Binder<Box<dyn Command>, I, A, D> {
        todo!()
    }

    fn bind(&self) -> Box<dyn Binding<C, I, A, D>> {
        todo!()
    }
}
