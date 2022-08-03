protocol AllMethods {
    // DELETE /delete
    func delete() async throws

	// GET /get
	func get() async throws

    // PATCH /patch
    func patch() async throws

    // POST /post
    func post() async throws

    // PUT /put
    func put() async throws
}
