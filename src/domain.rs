use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department_id: String,
    pub title: String,
    pub manager_id: Option<String>,
    pub start_date: NaiveDate,
    pub status: EmployeeStatus,
    pub salary: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmployeeStatus {
    Active,
    OnLeave,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: String,
    pub name: String,
    pub head_id: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOffRequest {
    pub id: String,
    pub employee_id: String,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: ApprovalStatus,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRecord {
    pub id: String,
    pub employee_id: String,
    pub period: String,
    pub gross: Money,
    pub deductions: Money,
    pub net: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount_minor: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgNode {
    pub employee_id: String,
    pub name: String,
    pub title: String,
    pub reports: Vec<OrgNode>,
}
