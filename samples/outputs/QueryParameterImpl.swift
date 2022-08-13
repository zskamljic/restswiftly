class QueryParameterImpl: QueryParameter {
    private let baseUrl: String
    private let interceptors: [Interceptor]

    init(baseUrl: String, interceptors: Interceptor...) {
        var baseUrl = baseUrl
        if baseUrl.hasSuffix("/") {
            baseUrl = String(baseUrl.removeLast())
        }
        self.baseUrl = baseUrl
        self.interceptors = interceptors
    }

    func get(query: String) async throws {
        var url = URL(string: baseUrl + "/get")!
        var urlComponents = URLComponents(string: url.absoluteString)!
        var queryItems = urlComponents.queryItems ?? []
        queryItems.append(URLQueryItem(name: "q", value: query))
        urlComponents.queryItems = queryItems
        url = urlComponents.url!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        let chain = Chain(using: interceptors) { URLSession.shared.data(for: request) }
        let (data, response) = try await chain.proceed(with: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        print(String(data: data, encoding: .utf8)!)
    }

    func get(for query: String) async throws {
        var url = URL(string: baseUrl + "/get")!
        var urlComponents = URLComponents(string: url.absoluteString)!
        var queryItems = urlComponents.queryItems ?? []
        queryItems.append(URLQueryItem(name: "q", value: query))
        queryItems.append(URLQueryItem(name: "q2", value: "something"))
        urlComponents.queryItems = queryItems
        url = urlComponents.url!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        let chain = Chain(using: interceptors) { URLSession.shared.data(for: request) }
        let (data, response) = try await chain.proceed(with: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        print(String(data: data, encoding: .utf8)!)
    }

}
