const path = require('path');
const fs = require('fs-extra');
const { Octokit } = require('@octokit/rest')
const { name, version } = require('../package.json')

const token = process.env.GITHUB_TOKEN;

const owner = "idan-at";
const repo = "native-sizeof"

async function publish() {
  const octokit = new Octokit({
    auth: token,
  });

  const { data: releases } = await octokit.repos.listReleases({
    owner,
    repo,
  });

  let release = releases.find(release => release.tag_name === version);

  if (!release) {
    ({ data: release }) = await octokit.repos.createRelease({
      owner,
      repo,
      tag_name: version,
      name: `v${version}`,
      body: `${name} ${version}`,
      prerelease: false
    });
  }

  const assetsPath = path.resolve(__dirname, "..", "build", "stage", version);

  const assets = await fs.readdir(assetsPath);

  await Promise.all(assets.map(async (asset) => {
    const data = await fs.readFile(path.join(assetsPath, asset));

    return octokit.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id: release.id,
      name: asset,
      data,
    });
  }));
};

if (require.main === module) {
  publish();
}
