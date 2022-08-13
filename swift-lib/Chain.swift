final class Chain {
    typealias ExchangeCall = (URLRequest) async throws -> (Data, URLResponse)

    private let interceptors: [any Interceptor]
    private let exchange: ExchangeCall
    private var currentInterceptor = 0

    init(using interceptors: [Interceptor], and exchange: @escaping ExchangeCall) {
        self.interceptors = interceptors
        self.exchange = exchange
    }

    func proceed(with request: URLRequest) async throws -> (Data, URLResponse) {
        if currentInterceptor == interceptors.count {
            return exchange(request)
        }
        let current = currentInterceptor;
        currentInterceptor += 1
        return interceptors[current].intercept(chain: self, for: request)
    }
}