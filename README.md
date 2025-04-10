# regret-log

Because your therapist doesn't need to know *everything*.

## What is this?

A minimalist CLI tool that lets you document your regrets from the comfort of your terminal, where no one can judge you except your compiler.

## Features

- Log your regrets without the awkward eye contact
- Tag entries like "bad_decision", "hold_my_beer", or "monday_morning"
- Track your mood on a scale from "existential dread" to "mildly disappointed"
- Statistics to confirm that, yes, most of your regrets do happen after 2am

## Installation

```bash
# Clone the repository of shame
git clone https://github.com/zacblev1/regret-log.git
cd regret-log

# Build and install your digital confessional
cargo install --path .
```

## Usage

```bash
# Document your latest questionable life choice
regret-log now

# Revisit your past mistakes (because you clearly haven't learned)
regret-log review

# Analyze the patterns of your poor judgment
regret-log stats
```

## How It Works

- Entries are stored in `~/.regret-log/log.yaml` (hidden, like your feelings)
- All data stays on your machine (your secrets are safe with us)
- Uses a terminal UI because GUIs are for people with fewer regrets

## Privacy Policy

What happens in the terminal, stays in the terminal.

## License

MIT (More Introspective Thoughts)