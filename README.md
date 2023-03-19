# OpenAI Buddy

This is a simple cli utility to communicate to OpenAI API.  It can help in testing or creating various models.  It provides full feature interfaces access to the completions, edits, audio, and images endpoints.

## Build
```bash
make
```

## Install
```bash
sudo make install
```

## Usage
To view the interface options anytime use the `-h/--help` flag.
```
oai [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
-v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)
    --stream     Stream back partial progress
-e, --echo       Echo back the prompt in addition
-h, --help       Prints help information

OPTIONS:
-m, --model <model>                            ID of the model to use [default: text-davinci-003]
    --max-tokens <max-tokens>                  The maximum number of tokens [default: 2048]
-t, --temperature <temperature>                What sampling temperature to use, between 0 and 2 [default: 0.5]
-a, --api-auth-token <api-auth-token>          API Authorization Token [env: API_AUTH_TOKEN]
-u, --user <user>                              User ID (default: session username)
-s, --suffix <suffix>                          After a completion of inserted text
    --top-p <top-p>                            Alternative to sampling with temperature [default: 1]
-n, --n <n>                                    How many completions to generate [default: 1]
-l, --logprobs <logprobs>                      Probabilities most likely tokens, as well the chosen tokens
    --stop <stop>...                           Returned text will not contain the stop sequence
-p, --presence-penalty <presence-penalty>
        Penalize new tokens based on whether they appear in the text so far [default: 0]

-f, --frequency-penalty <frequency-penalty>
        Penalize new tokens based on their existing frequency in the text so far [default: 0]

-b, --best-of <best-of>                        Highest log probability per token [default: 1]
    --logit-bias <logit-bias>                  Likelihood of specified tokens appearing

ARGS:
<prompt>         Question
<instruction>    Instructions how to edit the prompt

SUBCOMMANDS:
models        List of usable models
files         List, upload or remove files for account
fine-tunes    List, create, or cancel fine-tune jobs
audio         Transcribe or translate audio to text
image         Generate new, edited or variation images
```

### OpenAI Authentication Configuration

An OpenAI API Authentication Token must be provided to use this utility.  This can be obtained with an [OpenAI Service Account](https://platform.openai.com/signup) in the navigation under "[User > Api Keys](https://platform.openai.com/account/api-keys)".  When you obtain your api-key you can set your shell up to configure an environment variable with your api-key or at runtime with a flag.

**Configure shell:**
Add the following to your shells source file `.zshrc` or `.bashrc`.
```bash
export API_AUTH_TOKEN="<Your Token Here>"
```

**Configure at runtime w/ flag:**
```bash
oai --api-auth-token "<Your Token Here>" "My question?"
```

## Examples
Examples are provided in the Makefile instructions. They can be triggered using `make examples`. The following example commands will be ran:
```bash
{ echo "Please review the code and tell me if there are mistakes:"; cat examples/index.js; } | oai
{ echo "Please give a coded modification with a generic in the following Rust module:"; cat examples/test.rs; } | oai
{ echo "Please give me a summary of the following conversation:"; cat examples/convo.txt; } | oai
oai -t 1.2 "Write an HTML component with shadow DOM that ingest a style object and data object to create a button that displays a modal with data and allows the user to escape modal with the keyboards to escape key or exit button." > examples/button.html
oai "Do you plan on becoming our overlord and supreme ruler?"
```

---
Copyright 2023 Brandon Laurence Clark

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
