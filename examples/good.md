# Scan result for aakatev/scratch:latest

![Prisma Cloud Logo](https://www.paloaltonetworks.com/content/dam/pan/en_US/images/logos/brand/prisma-primary-reversed/Prisma-logo-reversed.png)


## Vulnerabilities

Status: Passed :white_check_mark:

<details>
<summary>See details</summary>

| Id | Package | Version | Description | Severity | Grace Days |
| --- | --- | --- | --- | --- | --- |


</details>

## Compliance

Status: Passed :white_check_mark:

<details>
<summary>See details</summary>

| Title | Severity | Description |
| --- | --- | --- |
| (CIS_Docker_v1.3.1 - 4.1) Image should be created with a non-root user | high | It is a good practice to run the container as a non-root user, if possible. Though usernamespace mapping is now available, if a user is already defined in the container image, thecontainer is run as that user by default and specific user namespace remapping is notrequired  |
| (CIS_Docker_v1.3.1 - 4.6) Add HEALTHCHECK instruction to the container image | medium | One of the important security triads is availability. Adding HEALTHCHECK instruction to yourcontainer image ensures that the docker engine periodically checks the running containerinstances against that instruction to ensure that the instances are still working  |


</details>

## Links

- [View scan result in Prisma Cloud console](https://prismacloud.example.com/#!/monitor/vulnerabilities/images/ci?search=sha256%3A71de1148337f4d1845be01eb4caf15d78e4eb15a1ab96030809826698a5b7e30)
- [Do other stuff](https://example.com)
