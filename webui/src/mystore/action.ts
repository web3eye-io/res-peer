import axios, { AxiosResponse } from 'axios'

function doAction<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  success: (resp: MyResponse) => void) {
  const api = axios.create()
  api
    .post<MyRequest, AxiosResponse<MyResponse>>(url, req)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
    })
    .catch(() => {
      // TODO
    })
}
function doActionWithError<MyRequest, MyResponse> (
  url: string,
  req: MyRequest,
  success: (resp: MyResponse) => void,
  error: () => void) {
  const api = axios.create()
  api
    .post<MyRequest, AxiosResponse<MyResponse>>(url, req)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
    })
    .catch(() => {
      error()
    })
}

function doGet<MyRequest, MyResponse> (
  url: string,
  success: (resp: MyResponse) => void) {
  const api = axios.create()
  api
    .get<MyRequest, AxiosResponse<MyResponse>>(url)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
    })
    .catch(() => {
      // TODO
    })
}

function doGetWithError<MyRequest, MyResponse> (
  url: string,
  success: (resp: MyResponse) => void,
  error: () => void) {
  const api = axios.create()
  api
    .get<MyRequest, AxiosResponse<MyResponse>>(url)
    .then((response: AxiosResponse<MyResponse>) => {
      success(response.data)
    })
    .catch(() => {
      error()
    })
}

export {
  doAction,
  doActionWithError,
  doGet,
  doGetWithError
}
