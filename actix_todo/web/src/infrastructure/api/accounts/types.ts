export interface FetchAccountsRequest {
  page: number;
  page_size: number;
}
export interface FetchAccountsResponse {
  data: AccountDatum[];
  total: number;
}

export interface AccountDatum {
  id: number;
  username: string;
  created_at: string;
  updated_at: string;
}

export interface UpdateAccountRequest {
  new_username: string;
}
export type UpdateAccountResponse = AccountDatum;
