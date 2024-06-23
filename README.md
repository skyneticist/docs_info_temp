# Notes

## iterate manifest repos
## check if new or existing

### if new --> we need to create an mdbook project for team/project
### init and push empty mdbook project to team repos
### if existing --> we need to check the diff

## parse repo contents
## sanitize contents
## tokenize contents
## generate embeddings
## build FileInfos

## generate documentation
## package documentation for local project/team mdbook (team-/project-level)
## package documentation for mdbook SWA (org-level)
## push packaged documentation to both repos

## local mdbook `src` directory === org-level docs SWA `chapter` directory (chapter is the team, sub-chapters projects, sub-sub-chapters are the folders and files of docs for project)
## The documentation is assembled/packaged in one-shot.
## Then, for each project's local mdbook that is packaged, there is a separate process that clones and renames `src` directory to `Project_Name` and pushes those contents to the org-level SWA repo's `src` dir as a `chapter` (contains many sub-chapters)

## need to include mdbook for central swa
## this is to maintain the mdbook features as dynamic content is added
## otherwise, would have to handle a lot of things manually!

#                                                                                            |--> push new `src` changes to project mdbook repo
# ORG_SWA_REPO --> manifest --> projects --> generate docs --> src/docs --> package docs --> |                                                                                       |
#                                                                                            |--> push new `chapter` changes to Vizient Docs repo

# This implies the need for the previously created then abandoned local mdbook and pipeline generation (as well as putting it in place) script on work machine

# avtodocs app will handle almost everything. meaning it will:

    - iterate repos
    - spawn async process per repo:
        - create dir
        - clone repo
        - iterate all files in all dirs
        - spawn async process per file:
            - pre-process file
            - generate docs chapter/sub-chapter
            - store generated file in dir
        - package all generated files in dir
        - add/edit contents to both remote repos (local and global mdbook projects)
    - Let both projects build (initiated by avtodocs app but handled by pipeline files and configurations)
    - fin
