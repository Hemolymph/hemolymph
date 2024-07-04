stripintegrity:
	@echo "Stripping integrity hashes from the generated index file"
	perl -pi -e 's/ *?integrity *?= *?\".*?\"//' ./dist/index.html
	@echo "Stripping integrity hashes from the generated index file"
	perl -pi -e 's/ *?integrity *?= *?\".*?\"//' ../server/dist/index.html

build:
	@echo "Building"
	trunk build --release
	cp -r ./dist ../server

builddebug:
	@echo "Building"
	trunk build
	cp -r ./dist ../server

postbuild: stripintegrity

run: build postbuild

debug: builddebug postbuild

.DEFAULT_GOAL := run
