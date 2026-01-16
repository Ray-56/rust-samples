import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthService } from "@/domains/auth/services/AuthService";
import { getToken, removeToken } from "@/infrastructure/storage/tokenStorage";

export function useAuth() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState<{ id: string; username: string } | null>(
    null
  );
  const navigate = useNavigate();

  useEffect(() => {
    const token = getToken();
    if (token) {
      setIsAuthenticated(true);
    }
  }, []);

  async function login(username: string, password: string) {
    setLoading(true);
    try {
      const userEntity = await AuthService.login(username, password).finally(
        () => {
          setLoading(false);
        }
      );
      setIsAuthenticated(true);
      setUser({
        id: userEntity.getId(),
        username: userEntity.getUsername().getValue(),
      });
      navigate("/todos");
    } catch (error) {
      throw new Error("Login failed. Error: " + error);
    }
  }

  async function register(username: string, password: string) {
    try {
      const userEntity = await AuthService.register(username, password);
      setIsAuthenticated(true);
      setUser({
        id: userEntity.getId(),
        username: userEntity.getUsername().getValue(),
      });
      navigate("/todos");
    } catch (error) {
      throw new Error("Registration failed. Error: " + error);
    }
  }

  async function forgotPassword(username: string, newPassword: string) {
    try {
      await AuthService.forgotPassword(username, newPassword);
      navigate("/login");
    } catch (error) {
      throw new Error("Reset password failed. Error: " + error);
    }
  }

  function logout() {
    removeToken();
    setIsAuthenticated(false);
    setUser(null);
    navigate("/login");
  }

  return {
    isAuthenticated,
    loading,
    user,
    login,
    register,
    forgotPassword,
    logout,
  };
}
