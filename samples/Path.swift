protocol Path {
	// GET /{path}/get
	func get(path: String) async throws
}
