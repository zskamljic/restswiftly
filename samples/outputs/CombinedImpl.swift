class CombinedImpl: Combined {
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

    func post(for query: String, with path: String, body: Hello) async throws -> Hello {
        var url = URL(string: baseUrl + "/{path}".replacingOccurrences(of: "{path}", with: path))!
        var urlComponents = URLComponents(string: url.absoluteString)!
        var queryItems = urlComponents.queryItems ?? []
        queryItems.append(URLQueryItem(name: "q", value: query))
        urlComponents.queryItems = queryItems
        url = urlComponents.url!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        let encoder = JSONEncoder()
        request.httpBody = try encoder.encode(body)
        let chain = Chain(using: interceptors) { URLSession.shared.data(for: request) }
        let (data, response) = try await chain.proceed(with: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        let decoder = JSONDecoder()
        return try decoder.decode(Hello.self, from: data)
    }

}
