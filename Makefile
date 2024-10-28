install:
	python -m pip install --upgrade pip && \
	python -m pip install -r requirements.txt


format:
	black *.py #format all files	

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

test:
	python -m pytest -cov test_main.py

generate_and_push:
	python main.py
	git config --local user.email "action@github.com"
	git config --local user.name "GitHub Action"
	git pull
	git add .
	git commit -m "rerun push"
	git push

all: install format lint test generate_and_push