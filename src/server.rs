use crate::domain::*;
use crate::store::Store;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IdInput { pub id: String }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DeptInput { pub department_id: Option<String> }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateEmployeeInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department_id: String,
    pub title: String,
    pub manager_id: Option<String>,
    pub start_date: String,
    pub salary_minor: i64,
    pub currency: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RequestTimeOffInput {
    pub employee_id: String,
    pub leave_type: String,
    pub start_date: String,
    pub end_date: String,
    pub reason: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ApproveTimeOffInput {
    pub request_id: String,
    pub approved: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PayrollInput { pub period: Option<String> }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RunPayrollInput { pub period: String }

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateEmployeeInput {
    pub id: String,
    pub title: Option<String>,
    pub department_id: Option<String>,
    pub manager_id: Option<String>,
    pub salary_minor: Option<i64>,
}

#[derive(Clone)]
pub struct HrisServer {
    pub store: Store,
}

#[tool_router]
impl HrisServer {
    #[tool(description = "List all employees, optionally filtered by department")]
    async fn list_employees(&self, Parameters(input): Parameters<DeptInput>) -> String {
        let emps = self.store.employees.read().await;
        let filtered: Vec<&Employee> = match &input.department_id {
            Some(d) => emps.iter().filter(|e| &e.department_id == d).collect(),
            None => emps.iter().collect(),
        };
        serde_json::to_string_pretty(&filtered).unwrap()
    }

    #[tool(description = "Get employee details by ID")]
    async fn get_employee(&self, Parameters(input): Parameters<IdInput>) -> String {
        let emps = self.store.employees.read().await;
        match emps.iter().find(|e| e.id == input.id) {
            Some(e) => serde_json::to_string_pretty(e).unwrap(),
            None => format!("Employee {} not found", input.id),
        }
    }

    #[tool(description = "Create a new employee record")]
    async fn create_employee(&self, Parameters(input): Parameters<CreateEmployeeInput>) -> String {
        let id = format!("emp-{}", uuid::Uuid::new_v4().as_simple().to_string().get(..8).unwrap());
        let emp = Employee {
            id: id.clone(),
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            department_id: input.department_id,
            title: input.title,
            manager_id: input.manager_id,
            start_date: chrono::NaiveDate::parse_from_str(&input.start_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            status: EmployeeStatus::Active,
            salary: Money { amount_minor: input.salary_minor, currency: input.currency.unwrap_or("USD".into()) },
        };
        self.store.employees.write().await.push(emp);
        format!("Created employee {}", id)
    }

    #[tool(description = "Update employee fields (title, department, manager, salary)")]
    async fn update_employee(&self, Parameters(input): Parameters<UpdateEmployeeInput>) -> String {
        let mut emps = self.store.employees.write().await;
        match emps.iter_mut().find(|e| e.id == input.id) {
            Some(e) => {
                if let Some(t) = input.title { e.title = t; }
                if let Some(d) = input.department_id { e.department_id = d; }
                if let Some(m) = input.manager_id { e.manager_id = Some(m); }
                if let Some(s) = input.salary_minor { e.salary.amount_minor = s; }
                format!("Updated employee {}", input.id)
            }
            None => format!("Employee {} not found", input.id),
        }
    }

    #[tool(description = "List all departments")]
    async fn list_departments(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        let depts = self.store.departments.read().await;
        serde_json::to_string_pretty(&*depts).unwrap()
    }

    #[tool(description = "Get department details by ID")]
    async fn get_department(&self, Parameters(input): Parameters<IdInput>) -> String {
        let depts = self.store.departments.read().await;
        match depts.iter().find(|d| d.id == input.id) {
            Some(d) => serde_json::to_string_pretty(d).unwrap(),
            None => format!("Department {} not found", input.id),
        }
    }

    #[tool(description = "Request time off for an employee")]
    async fn request_time_off(&self, Parameters(input): Parameters<RequestTimeOffInput>) -> String {
        let id = format!("pto-{}", uuid::Uuid::new_v4().as_simple().to_string().get(..8).unwrap());
        let req = TimeOffRequest {
            id: id.clone(),
            employee_id: input.employee_id,
            leave_type: input.leave_type,
            start_date: chrono::NaiveDate::parse_from_str(&input.start_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            end_date: chrono::NaiveDate::parse_from_str(&input.end_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            status: ApprovalStatus::Pending,
            reason: input.reason,
        };
        self.store.time_off.write().await.push(req);
        format!("Time-off request {} created (pending approval)", id)
    }

    #[tool(description = "List time-off requests, optionally for a specific employee")]
    async fn list_time_off(&self, Parameters(input): Parameters<DeptInput>) -> String {
        let reqs = self.store.time_off.read().await;
        let filtered: Vec<&TimeOffRequest> = match &input.department_id {
            Some(eid) => reqs.iter().filter(|r| &r.employee_id == eid).collect(),
            None => reqs.iter().collect(),
        };
        serde_json::to_string_pretty(&filtered).unwrap()
    }

    #[tool(description = "Approve or deny a time-off request")]
    async fn approve_time_off(&self, Parameters(input): Parameters<ApproveTimeOffInput>) -> String {
        let mut reqs = self.store.time_off.write().await;
        match reqs.iter_mut().find(|r| r.id == input.request_id) {
            Some(r) => {
                r.status = if input.approved { ApprovalStatus::Approved } else { ApprovalStatus::Denied };
                format!("Request {} {}", input.request_id, if input.approved { "approved" } else { "denied" })
            }
            None => format!("Request {} not found", input.request_id),
        }
    }

    #[tool(description = "List payroll records, optionally filtered by period (e.g. 2026-04)")]
    async fn list_payroll(&self, Parameters(input): Parameters<PayrollInput>) -> String {
        let records = self.store.payroll.read().await;
        let filtered: Vec<&PayrollRecord> = match &input.period {
            Some(p) => records.iter().filter(|r| &r.period == p).collect(),
            None => records.iter().collect(),
        };
        serde_json::to_string_pretty(&filtered).unwrap()
    }

    #[tool(description = "Run payroll for a period — generates pay records for all active employees")]
    async fn run_payroll(&self, Parameters(input): Parameters<RunPayrollInput>) -> String {
        let emps = self.store.employees.read().await;
        let active: Vec<&Employee> = emps.iter().filter(|e| matches!(e.status, EmployeeStatus::Active)).collect();
        let mut records = self.store.payroll.write().await;
        let mut count = 0;
        for emp in &active {
            let monthly_gross = emp.salary.amount_minor / 12;
            let deductions = monthly_gross * 25 / 100;
            records.push(PayrollRecord {
                id: format!("pay-{}", uuid::Uuid::new_v4().as_simple().to_string().get(..8).unwrap()),
                employee_id: emp.id.clone(),
                period: input.period.clone(),
                gross: Money { amount_minor: monthly_gross, currency: emp.salary.currency.clone() },
                deductions: Money { amount_minor: deductions, currency: emp.salary.currency.clone() },
                net: Money { amount_minor: monthly_gross - deductions, currency: emp.salary.currency.clone() },
            });
            count += 1;
        }
        format!("Payroll run for {}: {} employees processed", input.period, count)
    }

    #[tool(description = "Get the org chart as a tree structure")]
    async fn get_org_chart(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        let emps = self.store.employees.read().await;
        let tree = self.store.build_org_tree(&emps, None);
        serde_json::to_string_pretty(&tree).unwrap()
    }

    #[tool(description = "Get headcount summary by department")]
    async fn get_headcount(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        let emps = self.store.employees.read().await;
        let depts = self.store.departments.read().await;
        let summary: Vec<serde_json::Value> = depts.iter().map(|d| {
            let count = emps.iter().filter(|e| e.department_id == d.id && matches!(e.status, EmployeeStatus::Active)).count();
            serde_json::json!({ "department": d.name, "headcount": count })
        }).collect();
        serde_json::to_string_pretty(&summary).unwrap()
    }

    #[tool(description = "Get employee directory (name, email, title, department)")]
    async fn get_directory(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        let emps = self.store.employees.read().await;
        let depts = self.store.departments.read().await;
        let entries: Vec<serde_json::Value> = emps.iter().filter(|e| matches!(e.status, EmployeeStatus::Active)).map(|e| {
            let dept_name = depts.iter().find(|d| d.id == e.department_id).map(|d| d.name.as_str()).unwrap_or("Unknown");
            serde_json::json!({ "name": format!("{} {}", e.first_name, e.last_name), "email": e.email, "title": e.title, "department": dept_name })
        }).collect();
        serde_json::to_string_pretty(&entries).unwrap()
    }
}

adk_mcp_sdk::mcp_2026_server! {
    server: HrisServer,
    task_tools: ["run_payroll"],
    approval_tools: ["create_employee", "update_employee", "request_time_off", "approve_time_off", "run_payroll"],
    cache_ttl_ms: 60_000,
}
