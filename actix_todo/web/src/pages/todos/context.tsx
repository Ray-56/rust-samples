import { createContext } from "react";
import Todo from "@/common/domains/todo-domain/entities/todo";
import TodoService from "../../common/domains/todo-domain/todoService";

interface TodosCtxValue {
  dispatch: React.Dispatch<Actions>;
  state: State;
}
export const TodosCtx = createContext<TodosCtxValue>({} as TodosCtxValue);

export enum ActionType {
  SetupTodos,
  AddTodo,
  PatchTodo,
  DeleteTodo,
  RepositionTodo,
}

interface SetupTodos {
  type: ActionType.SetupTodos;
  payload: { pending: Todo[]; doing: Todo[]; completed: Todo[] };
}
interface AddTodo {
  type: ActionType.AddTodo;
  description: string;
}
interface PatchTodo {
  type: ActionType.PatchTodo;
  payload: {
    oldTodo: Todo;
    newTodo: Todo;
  };
}
interface DeleteTodo {
  type: ActionType.DeleteTodo;
  todo: Todo;
}
interface RepositionTodo {
  type: ActionType.RepositionTodo;
  // todo: Todo;
  payload: { type: API.TodoDatum["status"]; todos: Todo[] };
}

type Actions = AddTodo | PatchTodo | DeleteTodo | RepositionTodo | SetupTodos;

interface State {
  todoServer: TodoService;
  todos: {
    pending: Todo[];
    doing: Todo[];
    completed: Todo[];
  };
}

export const initializerState: State = {
  todoServer: new TodoService(),
  todos: {
    pending: [],
    doing: [],
    completed: [],
  },
};

export const reducer = (state: State, action: Actions) => {
  switch (action.type) {
    case ActionType.SetupTodos:
      return { ...state, todos: { ...action.payload } };
    case ActionType.AddTodo:
      const pending = state.todoServer.getPendingTodos();
      return { ...state, todos: { ...state.todos, pending } };
    case ActionType.PatchTodo: {
      const { newTodo, oldTodo } = action.payload;
      let todos = { ...state.todos };
      switch (newTodo?.getStatus()) {
        case "pending":
          const pending = state.todoServer.getPendingTodos();
          todos.pending = pending;
          break;
        case "doing":
          const doing = state.todoServer.getDoingTodos();
          todos.doing = doing;
          break;
        case "completed":
          const completed = state.todoServer.getCompletedTodos();
          todos.completed = completed;
          break;
      }
      switch (oldTodo.getStatus()) {
        case "pending":
          const pending = state.todoServer.getPendingTodos();
          todos.pending = pending;
          break;
        case "doing":
          const doing = state.todoServer.getDoingTodos();
          todos.doing = doing;
          break;
        case "completed":
          const completed = state.todoServer.getCompletedTodos();
          todos.completed = completed;
          break;
      }

      return { ...state, todos };
    }
    case ActionType.DeleteTodo: {
      switch (action.todo.getStatus()) {
        case "pending":
          const pending = state.todoServer.getPendingTodos();
          return { ...state, todos: { ...state.todos, pending } };
        case "doing":
          const doing = state.todoServer.getDoingTodos();
          return { ...state, todos: { ...state.todos, doing } };
        case "completed":
          const completed = state.todoServer.getCompletedTodos();
          return { ...state, todos: { ...state.todos, completed } };
      }
      return state;
    }
    case ActionType.RepositionTodo:
      const { type, todos } = action.payload;
      switch (type) {
        case "pending": {
          return { ...state, todos: { ...state.todos, pending: todos } };
        }
        case "doing":
          return { ...state, todos: { ...state.todos, doing: todos } };
        case "completed":
          return { ...state, todos: { ...state.todos, completed: todos } };
      }
      return state;
    default:
      const _: never = action;
      return _;
  }
};
