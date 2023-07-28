fn main() {
    
    let salary: f32 = 47000.0;
    let pension_rate: f32 = 0.03;
    let other_taxable: f32 = 5500.0;

    const MARRIAGE_TAX_ALLOWANCE: f32 = 1260.0; // 22/23 marriage allowance (make 0 if not applicable)
    const LOW_TAX_THRESHOLD: f32 = 12570.0; // 22/23 20% tax threshold
    const UPPER_TAX_THRESHOLD: f32 = 50271.0; // 22/23 40% tax threshold
    const STUDENT_LOAN_THRESHOLD: f32 = 27295.0; // 22/23 student loan threshold plan 2
    const STUDENT_LOAN_RATE: f32 = 0.09; // 22/23 student loan rate plan 2
    const MASTERS_LOAN_THRESHOLD: f32 = 21000.0;
    const MASTERS_LOAN_RATE: f32 = 0.06;
    const NI_RATE: f32 = 0.12; // 22/23 NI rate
    const NI_THRESHOLD: f32 = 12576.0; // 22/23 NI threshold
    const CHILD_TAX_THRESHOLD: f32 = 50000.0;
    const CHILD_TAX_BENEFIT: f32 = 1924.0;

    gross_income(salary, pension_rate, other_taxable);
    fn gross_income(salary: f32, pension_rate: f32, other_taxable: f32) {
        let pension: f32 = salary * pension_rate;
        let gross: f32 = salary + other_taxable - pension;
        print!("{gross}");
    }

    fn lower_tax(gross: f32) {
        
        let tax: f32 = (gross - LOW_TAX_THRESHOLD - MARRIAGE_TAX_ALLOWANCE) * 0.2;
    }


}
