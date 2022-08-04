class ReturnImpl: Return {
    func get() async throws -> Hello {
        let url = URL("https://httpbin.org/get")!
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
