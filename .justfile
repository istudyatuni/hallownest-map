port := "3000"

serve path=".":
	@echo Serving at http://localhost:{{ port }}/
	static-web-server -d "{{ path }}" -p {{ port }} --directory-listing
