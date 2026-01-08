# Git Multi-Account Setup

Managing work (Yahoo) and personal (GitHub) accounts on macOS.

## Directory Structure

```
~/
├── git/              # Work repos (Yahoo)
├── cursor/           # Work repos (Yahoo)
└── personal/         # Personal repos (GitHub - niwhsa)
```

## SSH Keys

| Account | Host | Key |
|---------|------|-----|
| Yahoo (`asuresh01`) | `git.ouryahoo.com` | `~/.ssh/id_rsa` |
| GitHub (`niwhsa`) | `github.com-niwhsa` | `~/.ssh/id_ed25519_niwhsa` |

### Generate new key (if needed)

```bash
ssh-keygen -t ed25519 -C "your-email@example.com" -f ~/.ssh/id_ed25519_<name>
```

### SSH Config (`~/.ssh/config`)

```ssh-config
# Work - Yahoo Git
Host git.ouryahoo.com
  HostName git.ouryahoo.com
  User git
  IdentityFile ~/.ssh/id_rsa
  IdentitiesOnly yes

# Personal GitHub (niwhsa)
Host github.com-niwhsa
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_niwhsa
  IdentitiesOnly yes
```

### Test SSH connections

```bash
ssh -T git@git.ouryahoo.com      # Should show: asuresh01
ssh -T git@github.com-niwhsa     # Should show: niwhsa
```

## Git Identity (Auto-switch by directory)

### `~/.gitconfig`

```gitconfig
[user]
    name = asuresh01
    email = ashwin.suresh@yahooinc.com

[includeIf "gitdir:~/personal/"]
    path = ~/.gitconfig-personal
```

### `~/.gitconfig-personal`

```gitconfig
[user]
    name = Ashwin Suresh
    email = ashwin.ms@gmail.com
```

### Verify identity

```bash
cd ~/git && git config user.email        # ashwin.suresh@yahooinc.com
cd ~/personal && git config user.email   # ashwin.ms@gmail.com
```

## Common Commands

### Clone personal repo

```bash
cd ~/personal
git clone git@github.com-niwhsa:niwhsa/<repo>.git
```

### Clone work repo

```bash
cd ~/git
git clone git@git.ouryahoo.com:netcloud/<repo>.git
```

### Create new personal repo

1. Create on https://github.com/new
2. Clone or init locally:

```bash
cd ~/personal
mkdir myrepo && cd myrepo
git init
git remote add origin git@github.com-niwhsa:niwhsa/myrepo.git
git push -u origin main
```

