# Scan result for aakatev/renovate:alpine

![Prisma Cloud Logo](https://www.paloaltonetworks.com/content/dam/pan/en_US/images/logos/brand/prisma-primary-reversed/Prisma-logo-reversed.png)


## Vulnerabilities

Status: Failed :x:

<details>
<summary>See details</summary>

| Id | Package | Version | Description | Severity | Grace Days |
| --- | --- | --- | --- | --- | --- |
| [CVE-2022-28391](https://nvd.nist.gov/vuln/detail/CVE-2022-28391) | busybox | 1.34.1-r4 | BusyBox through 1.35.0 allows remote attackers to execute arbitrary code if netstat is used to print a DNS PTR record\'s value to a VT compatible terminal. Alternatively, the attacker could choose to change the terminal\'s colors. | critical | -47 |
| [CVE-2022-24765](https://nvd.nist.gov/vuln/detail/CVE-2022-24765) | git | 2.34.1-r0 | Git for Windows is a fork of Git containing Windows-specific patches. This vulnerability affects users working on multi-user machines, where untrusted parties have write access to the same hard disk. Those untrusted parties could create the folder `C:\\.git`, which would be picked up by Git operations run supposedly outside a repository while searching for a Git directory. Git would then respect any config in said Git directory. Git Bash users who set `GIT_PS1_SHOWDIRTYSTATE` are vulnerable as well. Users who installed posh-gitare vulnerable simply by starting a PowerShell. Users of IDEs such as Visual Studio are vulnerable: simply creating a new project would already read and respect the config specified in `C:\\.git\\config`. Users of the Microsoft fork of Git are vulnerable simply by starting a Git Bash. The problem has been patched in Git for Windows v2.35.2. Users unable to upgrade may create the folder `.git` on all drives where Git commands are run, and remove read/write access from those folders as a workaround. Alternatively, define or extend `GIT_CEILING_DIRECTORIES` to cover the _parent_ directory of the user profile, e.g. `C:\\Users` if the user profile is located in `C:\\Users\\my-user-name`. | high | -15 |
| [CVE-2022-24785](https://github.com/advisories/GHSA-8hfj-j24r-96c4) | moment | 2.29.1 | Moment.js is a JavaScript date library for parsing, validating, manipulating, and formatting dates. A path traversal vulnerability impacts npm (server) users of Moment.js between versions 1.0.1 and 2.29.1, especially if a user-provided locale string is directly used to switch moment locale. This problem is patched in 2.29.2, and the patch can be applied to all affected versions. As a workaround, sanitize the user-provided locale name before passing it to Moment.js. | high | -23 |
| [CVE-2022-24066](https://nvd.nist.gov/vuln/detail/CVE-2022-24066) | simple-git | 3.4.0 | The package simple-git before 3.5.0 are vulnerable to Command Injection due to an incomplete fix of [CVE-2022-24433](https://security.snyk.io/vuln/SNYK-JS-SIMPLEGIT-2421199) which only patches against the git fetch attack vector. A similar use of the --upload-pack feature of git is also supported for git clone, which the prior fix didn\'t cover. | high | -25 |
| [CVE-2022-27776](https://nvd.nist.gov/vuln/detail/CVE-2022-27776) | curl | 7.80.0-r0 | DOCUMENTATION: No description is available for this CVE. | low | 150 |
| [CVE-2022-27775](https://nvd.nist.gov/vuln/detail/CVE-2022-27775) | curl | 7.80.0-r0 | Red Hat\'s versions of the associated software have been determined to NOT be affected by CVE-2022-27775. | low | 150 |
| [CVE-2022-27774](https://nvd.nist.gov/vuln/detail/CVE-2022-27774) | curl | 7.80.0-r0 | DOCUMENTATION: No description is available for this CVE. | low | 150 |
| [CVE-2022-22576](https://nvd.nist.gov/vuln/detail/CVE-2022-22576) | curl | 7.80.0-r0 | DOCUMENTATION: No description is available for this CVE. | low | 150 |
| [ALPINE-13661]() | busybox | 1.34.1-r4 |  | low | 0 |


</details>

## Compliance

Status: Passed :white_check_mark:

<details>
<summary>See details</summary>

| Title | Severity | Description |
| --- | --- | --- |
| (CIS_Docker_v1.3.1 - 4.1) Image should be created with a non-root user | high | It is a good practice to run the container as a non-root user, if possible. Though usernamespace mapping is now available, if a user is already defined in the container image, thecontainer is run as that user by default and specific user namespace remapping is notrequired  |
| Private keys stored in image | high |  Found: /usr/local/share/.cache/yarn/v6/npm-node-gyp-8.4.1-3d49308fc31f768180957d6b5746845fbd429937-integrity/node_modules/node-gyp/test/fixtures/server.key, /usr/local/share/.cache/yarn/v6/npm-node-gyp-9.0.0-e1da2067427f3eb5bb56820cb62bc6b1e4bd2089-integrity/node_modules/node-gyp/test/fixtures/server.key, /usr/local/share/.config/yarn/global/node_modules/node-gyp/test/fixtures/server.key |
| (CIS_Docker_v1.3.1 - 4.6) Add HEALTHCHECK instruction to the container image | medium | One of the important security triads is availability. Adding HEALTHCHECK instruction to yourcontainer image ensures that the docker engine periodically checks the running containerinstances against that instruction to ensure that the instances are still working  |


</details>

## Links

- [View scan result in Prisma Cloud console](https://example.com/#!/monitor/vulnerabilities/images/ci?search=sha256%3A47a63895a7e6ab14e5488fdb7a59165b6a95076bd82cffc61c622896c965ec45)
- [Do other stuff](https://example.com)
