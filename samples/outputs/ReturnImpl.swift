class ReturnImpl: Return {
    private let baseUrl: String

    init(baseUrl: String) {
        var baseUrl = baseUrl
        if baseUrl.hasSuffix("/") {
            baseUrl = String(baseUrl.removeLast())
        }
        self.baseUrl = baseUrl
    }

    func get() async throws -> Hello {
        let url = URL(string: baseUrl + "/get")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        let decoder = JSONDecoder()
        return try decoder.decode(Hello.self, from: data)
    }

}
