import request from "@/infrastructure/utils/request";
import {
  FetchAccountsRequest,
  FetchAccountsResponse,
  UpdateAccountRequest,
  UpdateAccountResponse,
} from "./types";

export const fetchAccounts = (params: FetchAccountsRequest) =>
  request
    .get<FetchAccountsResponse>("/accounts", { params })
    .then((res) => res.data);

export const updateAccount = (id: number, data: UpdateAccountRequest) =>
  request
    .patch<UpdateAccountResponse>(`/accounts/${id}`, data)
    .then((res) => res.data);
