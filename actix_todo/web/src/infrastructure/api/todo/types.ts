/**
 * @deprecated is not used temporarily because it is consistent with TodoDTO. It can be used later if the data returned by the API is inconsistent with TodoDTO. If used, it needs to be converted with TodoDTO in TodoService.createTodo
 */
export interface TodoApiDatum {}

export interface AddTodoRequest {
  description: string;
}
