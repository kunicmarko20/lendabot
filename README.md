Lendabot
========

Github and Slack bot that runs on AWS Lambda and automates workflow.

Github Commands
===============

All commands mentioned here should be executed as a comment to a Pull Request.


* `!ping` | `!p` - Check if Lendabot is alive.

* `!merge` | `!m` - Merges current Pull Request. By default Lendabot will use `squash and merge`.
If pull request is release (`development` into `master`) or a back-merge (`master` into `development`),
Lendabot will use `merge commit`.

* `!release` | `!r` - Creates a `development` into `master` Pull Request.

* `!update-release` | `!ur` - Updates the release Pull Request title with new ticket names.
Works only as a comment on a release Pull Request.

* `!back-merge` | `!bm` - Creates a `master` into `development` Pull Request. If Pull Request Event is enabled,
Lendabot will create this Pull Request automatically.

Slack Commands
==============

* `/release owner/repository` - Creates a `development` into `master` Pull Request.
