class MultipartEncodingImpl: MultipartEncoding {
    private let baseUrl: String

    init(baseUrl: String) {
        var baseUrl = baseUrl
        if baseUrl.hasSuffix("/") {
            baseUrl = String(baseUrl.removeLast())
        }
        self.baseUrl = baseUrl
    }

    func post(body: Hello) async throws {
        let url = URL(string: baseUrl + "/post")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("multipart/form-data", forHTTPHeaderField: "Content-Type")
        let encoder = MultipartEncoder()
        request.httpBody = try encoder.encode(body)
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        print(String(data: data, encoding: .utf8)!)
    }

}
