run:
	npm run-script serve

build:
	npm run-script build

check: 
	cargo watch --clear --exec check

test: 
	cargo watch --clear --ignore dump --shell "cargo test -- --nocapture"

crun:
	cargo watch --clear --exec run

cbuild:
	cargo watch --clear --exec build

publish:
	cargo build
	cargo publish

# # clean up feature branch BRANCH
# done BRANCH:
#   git checkout master
#   git diff --no-ext-diff --quiet --exit-code
#   git pull --rebase origin master
#   git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
#   git branch -D {{BRANCH}}
