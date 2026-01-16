/**
 * @deprecated 由于和 TodoDTO 一致，暂时不使用，后期如果 API 返回的数据和 TodoDTO 不一致，则可以使用。如果使用则需要在 TodoService.createTodo 中与 TodoDTO 进行转换
 */
export interface TodoApiDatum {}

export interface AddTodoRequest {
  description: string;
}
