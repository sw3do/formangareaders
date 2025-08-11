export type UserRole = 'user' | 'moderator' | 'admin'

export interface User {
  id: string
  email: string
  username: string
  display_name?: string
  avatar_url?: string
  role: UserRole
  is_verified: boolean
  provider: string
  locale: string
  created_at: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  username: string
  password: string
  display_name?: string
  locale?: string
}

export interface AuthResponse {
  user: User
  token: string
}

export interface ForgotPasswordRequest {
  email: string
}

export interface ResetPasswordRequest {
  token: string
  new_password: string
}

export interface VerifyEmailRequest {
  token: string
}