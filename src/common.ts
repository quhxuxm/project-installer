import {InjectionKey} from "vue";
import {ApplicationState} from "./messages/application_state.ts";

export const APPLICATION_STATE = Symbol() as InjectionKey<ApplicationState>