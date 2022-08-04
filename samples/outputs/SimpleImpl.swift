class SimpleImpl: Simple {
    func get() async throws {
        let url = URL("https://httpbin.org/get")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

}
