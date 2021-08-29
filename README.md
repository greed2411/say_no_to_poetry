# Say No To Poetry

Did you grow up using just using `pip` or `conda`? Are you being forced to use `poetry` everywhere at work?
They try to convince you `poetry` just works? You spiraled down into the conclusion that `poetry` is better than `pip`?

<p align="center">
<img src="https://media.giphy.com/media/LAKIIRqtM1dqE/giphy.gif?cid=ecf05e47mz6fgu03nmvmarvv9kj9vap8itua6ccy2vwkbh0k&rid=giphy.gif&ct=g" 
     width="480" height="358">
</p>

Its [dependency resolution is **shit** slow](https://python-poetry.org/docs/faq/#why-is-the-dependency-resolution-process-slow). If one of the package fails to install (yes looking at you uwsgi), `poetry` starts screaming like kid (printing out errors, which requires additional sheets), further won't install any of the other packages.

`poetry` is good only on good days, it is horrible on bad days. days where I need to `conda install -c conda-forge uwsgi` just for uwsgi. and since `poetry` gave up at `uwsgi`, I need to manually install other dependencies.

## So?

Use this binary instead,

```bash
say_no_to_poetry pyproject.toml
```

It converts your `pyproject.toml` into a `requirements.txt`.

### Example usage:

```bash
~/demo-sntp$ ls
hypothetical.pyproject.toml  say_no_to_poetry

~/demo-sntp$ cat hypothetical.pyproject.toml 
[tool.poetry]
name = "hypothetical"
version = "0.5.2"
description = "hypothetical utilities"
authors = []

[tool.poetry.dependencies]
python = "^3.8"
docopt = "^0.6.2"
scikit-learn = "^0.24.2"
stanza = { version = "^1.2", optional = true }
levenshtein = "^0.12.0"
tqdm = "^4.61.0"
pandas = "^1.3.1"
pydash = "^5.0.2"
furo = "^2021.4.11-beta.34"
pytz = "^2020.4"

[tool.poetry.extras]
asr = ["stanza"]

[tool.poetry.dev-dependencies]
pytest = "^6.2.4"
mypy = "^0.812"
jupyter = "^1.0.0"
black = "^20.8b1"


[build-system]
requires = ["poetry>=0.12"]
build-backend = "poetry.masonry.api"

~/demo-sntp$ ./say_no_to_poetry hypothetical.pyproject.toml 

~/demo-sntp$ ls
hypothetical.pyproject.toml  requirements.txt  say_no_to_poetry

~/demo-sntp$ cat requirements.txt 
black>=20.8b1
scikit-learn>=0.24.2
furo>=2021.4.11-beta.34
pytz>=2020.4
mypy>=0.812
pandas>=1.3.1
stanza>=1.2
tqdm>=4.61.0
pytest>=6.2.4
jupyter>=1.0.0
levenshtein>=0.12.0
docopt>=0.6.2
pydash>=5.0.2

```

## Caveats

Only after finishing this code, I realized there is an [`export` support](https://python-poetry.org/docs/cli/#export) in `poetry` itself.

So if you already have `poetry` globally/any-venv you can directly use it.

Also regarding this binary itself, complete syntax of how `version` can be defined is [here](https://python-poetry.org/docs/dependency-specification/) for `poetry`. the binary handles only the `^` and exact ones. not the `~`, `*` ones. Also doesn't set the upper bound.


## So is this useless?

Yeah whatever. It works, but pointless at this point of time. If you don't like `poetry`, don't have it in your system. use this binary, if you frequently need `requirements.txt` for setting up stuff.

It also helps in a situation where you want to use somebody's repo in your particular project/random environment. And if they happen to have `pyproject.toml`, convert it into `requirements.txt`, you can install dependencies without going through `poetry`, `poetry install` and `poetry` managing its own virtual environment for that repo.

tldr: you don't want to use `poetry`, happy with `pip`. Use this.

## Learnings

* my very first rust project.
* I should google more before starting a project. re-search.
* It could have been easier if I had taken time, to write down the steps and functions that'll make the project complete. And think if that's the most efficient way to do it. I ended up writing regex based file extraction (with look-aheads), to extract version for all possible `version` definitions. Without realzing it is just an ordinary `.toml` file. Later used the `toml` parser crate to do most of the hardwork.
* `lazy_static` is interesting, to avoid compiling regex patter several times, it is just done once during later stages while running. Therefore if there is a need to compile same pattern repeatedly, one can use this to save up runtime speed.
* I tried to write code such that each function is being told what to do directly, and given only the relevant information. no additional useless stuff.
