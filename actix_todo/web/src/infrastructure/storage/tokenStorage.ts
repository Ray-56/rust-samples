const TOKEN_KEY = "auth_token";

export function setToken(token: string): void {
  window.localStorage.setItem(TOKEN_KEY, token);
}

export function getToken(): string | null {
  return window.localStorage.getItem(TOKEN_KEY);
}

export function removeToken(): void {
  window.localStorage.removeItem(TOKEN_KEY);
}
