import {InjectionKey, Ref} from "vue";
import {ApplicationStateOutput} from "./messages/application_state_output.ts";

export const APPLICATION_STATE = Symbol() as InjectionKey<Ref<ApplicationStateOutput | undefined, ApplicationStateOutput | undefined>>

export const LOAD_APPLICATION_STATE_CMD = "load_application_state";

export const SAVE_GITHUB_CONFIG_CMD = "save_github_config";
export const SAVE_PROJECT_CONFIG_CMD = "save_project_config";

export const RETRIEVE_CODE_CMD = "retrieve_code";

export const SAVE_GITHUB_CONFIG_CMD_PARAM = "githubConfig";