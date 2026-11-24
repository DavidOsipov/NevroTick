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
            if (args.Length != 13 ||
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
                "failure=" + Describe(report.Failure),
                "recovery_failure=" + Describe(report.RecoveryFailure)
            }));

            return report.ExitCode == 0 && report.JobEmpty && !report.Recovered &&
                report.Failure == null && report.RecoveryFailure == null ? 0 : 1;
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
