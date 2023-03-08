export interface LoginForm {
    name: String,
    password: String
}

export interface UserInfo {
    roles?: Array<string>,
    username?: string,
    uuid?: string,
    headImg?: string,
    phone?: string,
    email?: string,
    addr?: string,
    userInde?: string,
}