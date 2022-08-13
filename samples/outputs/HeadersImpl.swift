class HeadersImpl: Headers {
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

    func get(for value: String) async throws {
        let url = URL(string: baseUrl + "/get")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.addValue(value, forHTTPHeaderField: "Custom")
        let chain = Chain(using: interceptors) { URLSession.shared.data(for: request) }
        let (data, response) = try await chain.proceed(with: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        print(String(data: data, encoding: .utf8)!)
    }

}
