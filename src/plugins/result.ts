export interface ResponseClass<T> {
    code: number,
    Data?: T
    msg?:string
}



export enum ResponseErrorType{
    SUCCESS = 200,
    ERROR_1 = -1, // 默认错误值
    IO_EXCEPTION=-2, // 文件异常
    NO_LOG = -10, // 没有登录
    NO_ACCESS = -11, //  没权限
    ERROR_100 = -100, // 未知
    
    ERROR_404 = 404,
    ERROR_503 = 503,

}