// SPDX-License-Identifier: GPL-2.0-only
using System;
using System.Globalization;
using System.Linq;
using System.Text.RegularExpressions;

namespace NevroTick
{
    // Dedicated fixture entry. All paths and pins come from the admitted caller.
    // Guardian output stays on the original pipes. This process only writes its
    // separate diagnostic record and returns the observed native exit decision.
    internal static class PrestartDeploymentWatchdog
    {
        private const string ObserveAccessSettlement = "--observe-access-settlement";
        private const int FailedAfterWriterSettlement = 3;

        private static int Main(string[] args)
        {
            long guardianBytes;
            long powerShellBytes;
            long ownerBytes;
            Guid run;
            int guardianWork;
            int guardianRetirement;
            int work;
            int retirement;
            bool observeAccessSettlement = args.Length == 14 &&
                String.Equals(args[13], ObserveAccessSettlement, StringComparison.Ordinal);
            if ((args.Length != 13 && !observeAccessSettlement) ||
                !PositiveLength(args[1], out guardianBytes) ||
                !PositiveLength(args[3], out powerShellBytes) ||
                !PositiveLength(args[6], out ownerBytes) ||
                !Guid.TryParseExact(args[8], "D", out run) ||
                run == Guid.Empty || !String.Equals(run.ToString("D"), args[8], StringComparison.Ordinal) ||
                !Budget(args[9], out guardianWork) || !Budget(args[10], out guardianRetirement) ||
                !Budget(args[11], out work) || !Budget(args[12], out retirement))
            {
                Console.Error.WriteLine("nevrotick-deployment-watchdog input-refused");
                return 2;
            }

            PrestartObserverChild.DeploymentJobReport report = PrestartObserverChild.RunDeploymentGuardianJob(
                args[0], guardianBytes, args[2], powerShellBytes, args[4], args[5], ownerBytes, args[7], run,
                guardianWork, guardianRetirement, work, retirement);
            // Restrict the record to checked identities and bounded diagnostic
            // fields. stderr from the guardian is separate from wire output.
            string members = report.MemberPidsAtRecovery == null ? "-" :
                String.Join(",", report.MemberPidsAtRecovery.Select(member => member.ToString(CultureInfo.InvariantCulture)));
            Console.Error.WriteLine("nevrotick-deployment-watchdog " + String.Join(" ", new string[] {
                "run=" + run.ToString("D"),
                "guardian_pid=" + Number(report.Pid),
                "guardian_exit=" + Number(report.ExitCode),
                "job_empty=" + Flag(report.JobEmpty),
                "recovered=" + Flag(report.Recovered),
                "active_at_recovery=" + Number(report.ActiveAtRecovery),
                "members_at_recovery=" + members,
                "total_processes=" + Number(report.TotalProcesses),
                "verified_process_limit=" + Number(report.VerifiedProcessLimit),
                "total_at_recovery=" + Number(report.TotalAtRecovery),
                "limit_terminations_at_recovery=" + Number(report.LimitTerminationsAtRecovery),
                "limit_terminations_at_empty=" + Number(report.LimitTerminationsAtEmpty),
                "recovery_start_tick=" + Number(report.RecoveryStartTick),
                "termination_accepted_tick=" + Number(report.TerminationAcceptedTick),
                "job_empty_tick=" + Number(report.JobEmptyTick),
                "tick_frequency=" + report.TickFrequency.ToString(CultureInfo.InvariantCulture),
                "failure=" + Describe(report.Failure),
                "recovery_failure=" + Describe(report.RecoveryFailure),
                "recovery_accounting_failure=" + Describe(report.RecoveryAccountingFailure),
                "recovery_members_failure=" + Describe(report.RecoveryMembersFailure)
            }));

            if (report.ExitCode == 0 && report.JobEmpty && !report.Recovered &&
                report.Failure == null && report.RecoveryFailure == null)
                return 0;
            // Preserve refusal while distinguishing a settled job for a private
            // diagnostic reader. Only a native observer of the full original exit
            // may interpret this value. Recovery and its diagnostics stay separate.
            bool failedAfterWriterSettlement = report.Pid.HasValue && report.ExitCode.HasValue &&
                report.ExitCode.Value != 0 && report.JobEmpty && report.RecoveryStartTick == null &&
                report.RecoveryFailure == null;
            return observeAccessSettlement && failedAfterWriterSettlement ? FailedAfterWriterSettlement : 1;
        }

        private static bool PositiveLength(string value, out long parsed)
        {
            return Int64.TryParse(value, NumberStyles.None, CultureInfo.InvariantCulture, out parsed) && parsed > 0;
        }

        private static bool Budget(string value, out int parsed)
        {
            return Int32.TryParse(value, NumberStyles.None, CultureInfo.InvariantCulture, out parsed) && parsed > 0;
        }

        private static string Flag(bool value)
        {
            return value ? "true" : "false";
        }

        private static string Number<T>(T? value) where T : struct, IFormattable
        {
            return value.HasValue ? value.Value.ToString(null, CultureInfo.InvariantCulture) : "-";
        }

        private static string Describe(Exception error)
        {
            if (error == null)
                return "-";
            const string native = @"\Aprestart-(deployment-job|observer|child):[a-z-]{1,48}\z";
            return Regex.IsMatch(error.Message, native) ? error.GetType().Name + "/" + error.Message :
                error.GetType().Name;
        }
    }
}
