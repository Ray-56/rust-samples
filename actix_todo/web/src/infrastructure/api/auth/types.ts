export interface LoginRequest {
  username: string;
  password: string;
}
export interface LoginResponse {
  token: string;
}

export interface RegisterAccountRequest {
  username: string;
  password: string;
}
export interface RegisterAccountResponse {
  id: number;
  username: string;
}

export interface ResetPasswordRequest {
  new_password: string;
}
export interface ResetPasswordResponse {
  message: string;
}
