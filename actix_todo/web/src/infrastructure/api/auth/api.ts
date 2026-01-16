import request from "@/infrastructure/utils/request";
import {
  LoginRequest,
  LoginResponse,
  RegisterAccountRequest,
  RegisterAccountResponse,
  ResetPasswordRequest,
  ResetPasswordResponse,
} from "./types";

export const login = (data: LoginRequest) =>
  request.post<LoginResponse>("/login", data).then((res) => res.data);

export const registerAccount = (data: RegisterAccountRequest) =>
  request
    .post<RegisterAccountResponse>("/accounts", data)
    .then((res) => res.data);

export const resetPasswordByUsername = (
  username: string,
  data: ResetPasswordRequest
) =>
  request
    .patch<ResetPasswordResponse>(`/accounts/reset_password/${username}`, data)
    .then((res) => res.data);
