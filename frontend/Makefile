stripintegrity:
	@echo "Stripping integrity hashes from the generated index file"
	perl -pi -e 's/ *?integrity *?= *?\".*?\"//' ./dist/index.html

build:
	@echo "Building"
	trunk build --release
	cp -r ./dist ../server/dist

builddebug:
	@echo "Building"
	trunk build
	cp -r ./dist ../server/dist

postbuild: stripintegrity

run: build postbuild

debug: builddebug postbuild

.DEFAULT_GOAL := run
