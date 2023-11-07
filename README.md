# trade-n-scrape-rs
### A webscraper for TradeMe built in Rust and 

Implementation plan for the Minimum Viable Product (MVP)

## 1. Scope Definition:

**Objective:** fetch data from trademe.co.nz based on user-defined categories, subcategories, and keywords to analyze pricing trends.

### **Core Features:**
- User input for categories, subcategories, and keywords.
- Data scraping from trademe.co.nz.
- Basic data storage.
- Simple analysis to identify pricing trends.
- Terminal-based user interface.

## 2. Technology Selection:

### **Languages:**

**Backend:** Rust (for efficient data scraping and processing).

**Frontend:** A simple CLI (Command Line Interface) initially, to be developed using a library such as clap in Rust.

### **Frameworks:**

**Web scraping:** reqwest and scraper libraries in Rust.
**Database

### **Containerization:**
Install Docker and learn the basics if you're not already familiar. Create a Dockerfile for your project that defines the environment needed to run your application.

Build a Docker image from this Dockerfile. Run your application within a Docker container during development to ensure consistency across different environments.

## 3. Architecture:

**Modular Architecture:** Break down the system into independent modules like:

1. **Input Module:** 
Handles user inputs.

2. **Scraper Module:** 
Handles data scraping.

3. **Storage Module:** 
Manages data storage.

4. **Analysis Module:** 
Conducts basic analysis.

## 4. Design Patterns:

**Singleton:** For managing database connections.

**Strategy:** To allow different scraping or analysis strategies.

## 5. Development Milestones:
**Week 1:**
Setup development environment.
Establish database schema.
Implement Input Module.

**Week 2:**
Implement Scraper Module.
Start with Storage Module.

**Week 3:**
Complete Storage Module.
Implement basic Analysis Module.

**Week 4:**
Refinement and testing.
Document the code and basic user guide.


## 6. Work Tracking:
Utilize GitHub Projects to set up a Kanban board for your project.
Create columns like Backlog, In Progress, Review, and Done to track the status of your tasks.
Create issues for each task or feature, and move them through the columns as you progress.
Use Milestones in GitHub to track progress towards major project checkpoints, like completing a module or achieving a working prototype.

## 7. Testing:

Unit testing each module using a Rust testing framework like cargo-test.
Simple end-to-end testing to ensure data flows correctly through the system.

## 8. Documentation:

Maintain a developer journal to document your progress, challenges, and learning.
Document the codebase adequately to understand the flow and logic.

Document your Docker setup and GitHub Actions workflows in your project’s README or a separate documentation directory.

Keep your Kanban board and GitHub Milestones updated to reflect your project’s progress and upcoming tasks.

## 9. Version Control:

Utilize GitHub for version control, tracking issues, and managing milestones.

Set up a .github/workflows directory in your repository. Create a workflow file (e.g., ci.yml) to define Continuous Integration (CI) tasks such as building your application, running tests, and pushing Docker images to a container registry.

Explore GitHub Actions Marketplace for actions that could automate repetitive tasks, like labeling issues, managing stale issues, or automating project board updates.

### **Integration:**
Link your Kanban board with GitHub Actions using automation tools or marketplace actions to automatically move issues through columns based on actions like PR merges or issue comments.

Use GitHub Actions to automatically update your project's Docker image whenever you push changes to specific branches, ensuring your containerized environment stays up-to-date.

## 10. Routine:

Dedicate specific time slots for development, testing, and documentation.
Regularly review the progress against the plan to stay on track.
This plan aims at a balance between structure and flexibility to accommodate learning and adjustments along the way. It’s structured to provide a clear path towards achieving a working MVP while leaving room for exploration and learning.

## 11. Review and Adaptation:
Periodically review your workflow, container setup, and automation pipelines. Adapt them as needed to suit the evolving requirements of your project.
Explore additional automation, container orchestration (e.g., Kubernetes), or CI/CD tools that could further streamline your development process.

