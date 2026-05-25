use crate::domain::*;
use chrono::NaiveDate;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub employees: Arc<RwLock<Vec<Employee>>>,
    pub departments: Arc<RwLock<Vec<Department>>>,
    pub time_off: Arc<RwLock<Vec<TimeOffRequest>>>,
    pub payroll: Arc<RwLock<Vec<PayrollRecord>>>,
}

impl Store {
    pub fn seeded() -> Self {
        let departments = vec![
            Department { id: "dept-1".into(), name: "Engineering".into(), head_id: Some("emp-1".into()), parent_id: None },
            Department { id: "dept-2".into(), name: "Product".into(), head_id: Some("emp-3".into()), parent_id: None },
            Department { id: "dept-3".into(), name: "HR".into(), head_id: Some("emp-5".into()), parent_id: None },
        ];
        let employees = vec![
            Employee { id: "emp-1".into(), first_name: "Alice".into(), last_name: "Chen".into(), email: "alice@company.com".into(), department_id: "dept-1".into(), title: "VP Engineering".into(), manager_id: None, start_date: NaiveDate::from_ymd_opt(2020, 3, 15).unwrap(), status: EmployeeStatus::Active, salary: Money { amount_minor: 2500000, currency: "USD".into() } },
            Employee { id: "emp-2".into(), first_name: "Bob".into(), last_name: "Smith".into(), email: "bob@company.com".into(), department_id: "dept-1".into(), title: "Senior Engineer".into(), manager_id: Some("emp-1".into()), start_date: NaiveDate::from_ymd_opt(2021, 6, 1).unwrap(), status: EmployeeStatus::Active, salary: Money { amount_minor: 1800000, currency: "USD".into() } },
            Employee { id: "emp-3".into(), first_name: "Carol".into(), last_name: "Davis".into(), email: "carol@company.com".into(), department_id: "dept-2".into(), title: "Head of Product".into(), manager_id: None, start_date: NaiveDate::from_ymd_opt(2019, 11, 1).unwrap(), status: EmployeeStatus::Active, salary: Money { amount_minor: 2200000, currency: "USD".into() } },
            Employee { id: "emp-4".into(), first_name: "Dan".into(), last_name: "Lee".into(), email: "dan@company.com".into(), department_id: "dept-1".into(), title: "Engineer".into(), manager_id: Some("emp-1".into()), start_date: NaiveDate::from_ymd_opt(2023, 1, 10).unwrap(), status: EmployeeStatus::Active, salary: Money { amount_minor: 1400000, currency: "USD".into() } },
            Employee { id: "emp-5".into(), first_name: "Eve".into(), last_name: "Johnson".into(), email: "eve@company.com".into(), department_id: "dept-3".into(), title: "HR Director".into(), manager_id: None, start_date: NaiveDate::from_ymd_opt(2020, 8, 20).unwrap(), status: EmployeeStatus::Active, salary: Money { amount_minor: 1900000, currency: "USD".into() } },
        ];
        let time_off = vec![
            TimeOffRequest { id: "pto-1".into(), employee_id: "emp-2".into(), leave_type: "Vacation".into(), start_date: NaiveDate::from_ymd_opt(2026, 6, 10).unwrap(), end_date: NaiveDate::from_ymd_opt(2026, 6, 14).unwrap(), status: ApprovalStatus::Approved, reason: Some("Family trip".into()) },
            TimeOffRequest { id: "pto-2".into(), employee_id: "emp-4".into(), leave_type: "Sick".into(), start_date: NaiveDate::from_ymd_opt(2026, 5, 20).unwrap(), end_date: NaiveDate::from_ymd_opt(2026, 5, 21).unwrap(), status: ApprovalStatus::Approved, reason: None },
        ];
        let payroll = vec![
            PayrollRecord { id: "pay-1".into(), employee_id: "emp-1".into(), period: "2026-04".into(), gross: Money { amount_minor: 2083333, currency: "USD".into() }, deductions: Money { amount_minor: 520833, currency: "USD".into() }, net: Money { amount_minor: 1562500, currency: "USD".into() } },
            PayrollRecord { id: "pay-2".into(), employee_id: "emp-2".into(), period: "2026-04".into(), gross: Money { amount_minor: 1500000, currency: "USD".into() }, deductions: Money { amount_minor: 375000, currency: "USD".into() }, net: Money { amount_minor: 1125000, currency: "USD".into() } },
        ];
        Self {
            employees: Arc::new(RwLock::new(employees)),
            departments: Arc::new(RwLock::new(departments)),
            time_off: Arc::new(RwLock::new(time_off)),
            payroll: Arc::new(RwLock::new(payroll)),
        }
    }

    pub fn build_org_tree(&self, employees: &[Employee], root_id: Option<&str>) -> Vec<OrgNode> {
        let roots: Vec<&Employee> = employees.iter().filter(|e| e.manager_id.as_deref() == root_id).collect();
        roots.iter().map(|e| OrgNode {
            employee_id: e.id.clone(),
            name: format!("{} {}", e.first_name, e.last_name),
            title: e.title.clone(),
            reports: self.build_org_tree(employees, Some(&e.id)),
        }).collect()
    }
}
