protocol Interceptor {
    func intercept(chain: Chain, for request: URLRequest) async throws -> (Data, URLResponse)
}