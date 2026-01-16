import request from "@/infrastructure/utils/request";
import { TodoDTO } from "@/domains/todo/types";
import { AddTodoRequest } from "./types";

export const getTodos = () =>
  request.get<TodoDTO[]>("/todos").then((res) => res.data);

export const addTodo = (data: AddTodoRequest) =>
  request.post<TodoDTO>("/todos", data).then((res) => res.data);

export const updateTodo = (
  id: number,
  data: Pick<TodoDTO, "description" | "position" | "status">
) => request.put<TodoDTO>(`/todos/${id}`, data).then((res) => res.data);

export const patchTodo = (
  id: number,
  data: Partial<Pick<TodoDTO, "description" | "position" | "status">>
) => request.patch<TodoDTO>(`/todos/${id}`, data).then((res) => res.data);

export const deleteTodo = (id: number) => request.delete(`/todos/${id}`);

export const reorderTodo = (status: TodoDTO["status"]) =>
  request
    .patch<TodoDTO[]>(`/todos/reorder`, { status })
    .then((res) => res.data);
