import { request } from "@/common/util";

export const getTodos = () => request.get<API.TodoDatum[]>("/todos");

export const addTodo = (data: API.AddTodoDatumPayload) =>
  request.post<API.TodoDatum>("/todos", data);

export const updateTodo = (id: number, data: API.TodoDatum) =>
  request.put(`/todos/${id}`, data);

export const patchTodo = (
  id: number,
  data: Partial<Omit<API.TodoDatum, "id">>
) => request.patch<API.TodoDatum>(`/todos/${id}`, data);

export const deleteTodo = (id: number) => request.delete(`/todos/${id}`);

export const reorderTodo = (status: API.TodoDatum["status"]) =>
  request.patch(`/todos/reorder`, { status });
